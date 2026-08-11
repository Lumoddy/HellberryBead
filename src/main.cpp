#include <Arduino.h>
#include "setup.h"
#include "result.h"
#include "incoming.h"
#include "outgoing.h"
#include "pin.h"
#include "state.h"

#ifdef BOARD_ARDUINO_UNO
DiDoState d2(2);
DiDoAoState d3(3);
DiDoState d4(4);
DiDoAoState d5(5);
DiDoAoState d6(6);
DiDoState d7(7);
DiDoState d8(8);
DiDoAoState d9(9);
DiDoAoState d10(10);
DiDoAoState d11(11);
DiDoState d12(12);
DiDoState d13(13);
DiDoAiState a0(A0);
DiDoAiState a1(A1);
DiDoAiState a2(A2);
DiDoAiState a3(A3);
DiDoAiState a4(A4);
DiDoAiState a5(A5);
#define FOREACH_PIN(...) \
    { DiDoState& pin = d2; __VA_ARGS__ } \
    { DiDoAoState& pin = d3; __VA_ARGS__ } \
    { DiDoState& pin = d4; __VA_ARGS__ } \
    { DiDoAoState& pin = d5; __VA_ARGS__ } \
    { DiDoAoState& pin = d6; __VA_ARGS__ } \
    { DiDoState& pin = d7; __VA_ARGS__ } \
    { DiDoState& pin = d8; __VA_ARGS__ } \
    { DiDoAoState& pin = d9; __VA_ARGS__ } \
    { DiDoAoState& pin = d10; __VA_ARGS__ } \
    { DiDoAoState& pin = d11; __VA_ARGS__ } \
    { DiDoState& pin = d12; __VA_ARGS__ } \
    { DiDoState& pin = d13; __VA_ARGS__ } \
    { DiDoAiState& pin = a0; __VA_ARGS__ } \
    { DiDoAiState& pin = a1; __VA_ARGS__ } \
    { DiDoAiState& pin = a2; __VA_ARGS__ } \
    { DiDoAiState& pin = a3; __VA_ARGS__ } \
    { DiDoAiState& pin = a4; __VA_ARGS__ } \
    { DiDoAiState& pin = a5; __VA_ARGS__ }
#else
#error "Unknown board type"
#endif

#define PACKET_CONTROL 16
#define PACKET_START 2

void setup()
{
    Serial.begin(9600);
    FOREACH_PIN(pin.begin();)
}

struct NewBegining { };

struct EscapedReaderError
{
public:
    Result<NewBegining, outgoing::Packet> inner;

    EscapedReaderError(NewBegining value) :
        inner(Result<NewBegining, outgoing::Packet>::makeOk(value)) { }

    EscapedReaderError(outgoing::Packet packet) :
        inner(Result<NewBegining, outgoing::Packet>::makeErr(packet)) { }

public:
    static EscapedReaderError invalidPacketId()
    {
        return EscapedReaderError
        {
            outgoing::Packet(outgoing::InvalidPacketId { }),
        };
    }

    static EscapedReaderError invalidPinMode()
    {
        return EscapedReaderError
        {
            outgoing::Packet(outgoing::InvalidPinMode { }),
        };
    }
};

struct EscapedReader
{
    Result<uint8_t, EscapedReaderError> operator()()
    {
        while (!Serial.available()) { }
        uint8_t byte0 = Serial.read();
        if (byte0 != PACKET_CONTROL)
            return byte0;

        while (!Serial.available()) { }
        uint8_t byte1 = Serial.read();
        switch (byte1)
        {
            case PACKET_CONTROL: return PACKET_CONTROL;
            case PACKET_START: return Result<uint8_t, EscapedReaderError>::makeErr(EscapedReaderError
            {
                NewBegining { },
            });
            default: return Result<uint8_t, EscapedReaderError>::makeErr(EscapedReaderError
            {
                outgoing::Packet(outgoing::InvalidEscape { }),
            });
        }
    }
};

struct EscapedWriter
{
    void start()
    {
        Serial.write(PACKET_CONTROL);
        Serial.write(PACKET_START);
    }

    void operator()(uint8_t byte)
    {
        if (byte == PACKET_CONTROL)
            Serial.write(PACKET_CONTROL);
        Serial.write(byte);
    }
};

void loop()
{
    EscapedReader reader;
    EscapedWriter writer;

    FOREACH_PIN(
    {
        Result<uint16_t, void> poll = pin.pollListen();
        if (poll.isOk())
        {
            writer.start();
            outgoing::serialize(
                writer,
                outgoing::Packet(outgoing::PinListen { *poll.ok() }));
        }
    })

    if (Serial.available())
    {
        Result<incoming::Packet, EscapedReaderError> item =
            incoming::deserializePacket<EscapedReaderError, EscapedReader>(reader);

        if (item.isErr())
        {
            if (item.err()->inner.isOk())
            {
                return;
            }
            else
            {
                writer.start();
                outgoing::serialize(writer, *item.err()->inner.err());
            }
        }
        else
        {
            incoming::Packet& incoming = *item.ok();

            if (incoming.isPing())
            {
                writer.start();
                outgoing::serialize(writer, outgoing::Packet(outgoing::Pong { }));
            }
            else if (incoming.isWholeConfig())
            {
                writer.start();
                outgoing::serialize(writer, outgoing::Packet(outgoing::Config { }));
            }
            else if (incoming.isGetPinPower())
            {
                uint8_t i = incoming.getPinPower()->pin;

                FOREACH_PIN(
                {
                    if (i == 0)
                    {
                        writer.start();
                        outgoing::serialize(
                            writer,
                            outgoing::Packet(outgoing::GetPinPowerResponse { pin.getPower() }));

                        return;
                    }

                    i -= 1;
                })

                writer.start();
                outgoing::serialize(writer, outgoing::Packet(outgoing::InvalidPinId { }));
            }
            else if (incoming.isGetPinMode())
            {
                uint8_t i = incoming.getPinMode()->pin;

                FOREACH_PIN(
                {
                    if (i == 0)
                    {
                        writer.start();
                        outgoing::serialize(
                            writer,
                            outgoing::Packet(outgoing::GetPinModeResponse { pin.getMode() }));

                        return;
                    }

                    i -= 1;
                })

                writer.start();
                outgoing::serialize(writer, outgoing::Packet(outgoing::InvalidPinId { }));
            }
            else if (incoming.isSetPinPower())
            {
                uint8_t i = incoming.setPinPower()->pin;
                uint8_t power = incoming.setPinPower()->power;

                FOREACH_PIN(
                {
                    if (i == 0)
                    {
                        const outgoing::Packet& response = pin.setPower(power)
                            .errOr(outgoing::Packet(outgoing::SetPinPowerResponse { }));

                        writer.start();
                        outgoing::serialize(writer, response);

                        return;
                    }

                    i -= 1;
                })

                writer.start();
                outgoing::serialize(writer, outgoing::Packet(outgoing::InvalidPinId { }));
            }
            else if (incoming.isSetPinMode())
            {
                uint8_t i = incoming.setPinMode()->pin;
                PinMode mode = incoming.setPinMode()->mode;

                FOREACH_PIN(
                {
                    if (i == 0)
                    {
                        const outgoing::Packet& response = pin.setMode(mode)
                            .errOr(outgoing::Packet(outgoing::SetPinModeResponse { }));

                        writer.start();
                        outgoing::serialize(writer, response);

                        return;
                    }

                    i -= 1;
                })

                writer.start();
                outgoing::serialize(writer, outgoing::Packet(outgoing::InvalidPinId { }));
            }
        }
    }
}