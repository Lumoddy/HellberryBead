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
#define FOREACH_PIN(pin, id, ...) \
    { \
        { [[maybe_unused]] DiDoState& pin = d2; [[maybe_unused]] uint8_t id = 0; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAoState& pin = d3; [[maybe_unused]] uint8_t id = 1; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoState& pin = d4; [[maybe_unused]] uint8_t id = 2; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAoState& pin = d5; [[maybe_unused]] uint8_t id = 3; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAoState& pin = d6; [[maybe_unused]] uint8_t id = 4; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoState& pin = d7; [[maybe_unused]] uint8_t id = 5; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoState& pin = d8; [[maybe_unused]] uint8_t id = 6; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAoState& pin = d9; [[maybe_unused]] uint8_t id = 7; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAoState& pin = d10; [[maybe_unused]] uint8_t id = 8; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAoState& pin = d11; [[maybe_unused]] uint8_t id = 9; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoState& pin = d12; [[maybe_unused]] uint8_t id = 10; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoState& pin = d13; [[maybe_unused]] uint8_t id = 11; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAiState& pin = a0; [[maybe_unused]] uint8_t id = 12; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAiState& pin = a1; [[maybe_unused]] uint8_t id = 13; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAiState& pin = a2; [[maybe_unused]] uint8_t id = 14; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAiState& pin = a3; [[maybe_unused]] uint8_t id = 15; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAiState& pin = a4; [[maybe_unused]] uint8_t id = 16; __VA_ARGS__ } \
        { [[maybe_unused]] DiDoAiState& pin = a5; [[maybe_unused]] uint8_t id = 17; __VA_ARGS__ } \
    }
#else
#error "Unknown board type"
#endif

#define PACKET_CONTROL 16
#define PACKET_START 2

void setup()
{
    Serial.begin(9600);
    FOREACH_PIN(pin, i,
    {
        pin.begin();
    })
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

    FOREACH_PIN(pin, i,
    {
        Result<uint16_t, void> poll = pin.pollListen();
        if (poll.isOk())
        {
            writer.start();
            outgoing::serialize(
                writer,
                outgoing::Packet(outgoing::PinListen { i, *poll.ok() }));
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
                uint8_t id = incoming.getPinPower()->pin;

                FOREACH_PIN(pin, i,
                {
                    if (id == i)
                    {
                        writer.start();
                        outgoing::serialize(
                            writer,
                            outgoing::Packet(outgoing::GetPinPowerResponse { id, pin.getPower() }));

                        return;
                    }
                })

                writer.start();
                outgoing::serialize(writer, outgoing::Packet(outgoing::InvalidPinId { id }));
            }
            else if (incoming.isGetPinMode())
            {
                uint8_t id = incoming.getPinMode()->pin;

                FOREACH_PIN(pin, i,
                {
                    if (id == i)
                    {
                        writer.start();
                        outgoing::serialize(
                            writer,
                            outgoing::Packet(outgoing::GetPinModeResponse { id, pin.getMode() }));

                        return;
                    }
                })

                writer.start();
                outgoing::serialize(writer, outgoing::Packet(outgoing::InvalidPinId { id }));
            }
            else if (incoming.isSetPinPower())
            {
                uint8_t id = incoming.setPinPower()->pin;
                uint8_t power = incoming.setPinPower()->power;

                FOREACH_PIN(pin, i,
                {
                    if (id == i)
                    {
                        Result<void, outgoing::Packet> result = pin.setPower(power);
                        if (result.isErr())
                        {
                            writer.start();
                            outgoing::serialize(writer, *result.err());
                        }
                        else
                        {
                            writer.start();
                            outgoing::serialize(writer, outgoing::Packet(outgoing::SetPinPowerResponse { id, power }));
                        }

                        return;
                    }
                })

                writer.start();
                outgoing::serialize(writer, outgoing::Packet(outgoing::InvalidPinId { id }));
            }
            else if (incoming.isSetPinMode())
            {
                uint8_t id = incoming.setPinMode()->pin;
                PinMode mode = incoming.setPinMode()->mode;

                FOREACH_PIN(pin, i,
                {
                    if (id == i)
                    {
                        Result<void, outgoing::Packet> result = pin.setMode(mode);
                        if (result.isErr())
                        {
                            writer.start();
                            outgoing::serialize(writer, *result.err());
                        }
                        else
                        {
                            writer.start();
                            outgoing::serialize(writer, outgoing::Packet(outgoing::SetPinModeResponse { id, mode }));
                        }

                        return;
                    }
                })

                writer.start();
                outgoing::serialize(writer, outgoing::Packet(outgoing::InvalidPinId { }));
            }
        }
    }
}