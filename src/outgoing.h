#ifndef outgoing_h
#define outgoing_h

#include <stdint.h>
#include "setup.h"
#include "pin.h"

namespace outgoing
{
    enum struct PacketType
    {
        Pong,
        Config,
        GetPinPowerResponse,
        GetPinModeResponse,
        SetPinPowerResponse,
        SetPinModeResponse,
        PinListen,
        InvalidPinMode,
        InvalidPinId,
        InvalidPacketId,
        InvalidWriteToInput,
        InvalidUnsupportedMode,
        InvalidEscape,
    };

    struct Pong
    {
        [[nodiscard]] static constexpr PacketType type() { return PacketType::Pong; }
    };

    struct Config
    {
        [[nodiscard]] static constexpr PacketType type() { return PacketType::Config; }
    };

    struct GetPinPowerResponse
    {
        uint8_t pin;
        uint16_t power;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::GetPinPowerResponse; }
    };

    struct GetPinModeResponse
    {
        uint8_t pin;
        PinMode mode;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::GetPinModeResponse; }
    };

    struct SetPinPowerResponse
    {
        uint8_t pin;
        uint16_t power;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::SetPinPowerResponse; }
    };

    struct SetPinModeResponse
    {
        uint8_t pin;
        PinMode mode;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::SetPinModeResponse; }
    };

    struct PinListen
    {
        uint8_t pin;
        uint16_t power;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::PinListen; }
    };

    struct InvalidPinMode
    {
        uint8_t byte;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::InvalidPinMode; }
    };

    struct InvalidPinId
    {
        uint8_t pin;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::InvalidPinId; }
    };

    struct InvalidPacketId
    {
        uint8_t byte;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::InvalidPacketId; }
    };

    struct InvalidWriteToInput
    {
        uint8_t pin;
        uint16_t power;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::InvalidWriteToInput; }
    };

    struct InvalidUnsupportedMode
    {
        uint8_t pin;
        PinMode mode;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::InvalidUnsupportedMode; }
    };

    struct InvalidEscape
    {
        uint8_t byte;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::InvalidEscape; }
    };

    struct Packet
    {
    private:
        typedef PacketType Type;

        union Data
        {
            Pong pong;
            Config config;
            GetPinPowerResponse getPinPowerResponse;
            GetPinModeResponse getPinModeResponse;
            SetPinPowerResponse setPinPowerResponse;
            SetPinModeResponse setPinModeResponse;
            PinListen pinListen;
            InvalidPinMode invalidPinMode;
            InvalidPinId invalidPinId;
            InvalidPacketId invalidPacketId;
            InvalidWriteToInput invalidWriteToInput;
            InvalidUnsupportedMode invalidUnsupportedMode;
            InvalidEscape invalidEscape;
        };

        Type tag;
        Data data;

    public:
        constexpr Packet(Pong value) : tag(Type::Pong), data(Data { .pong = value }) { }
        constexpr Packet(Config value) : tag(Type::Config), data(Data { .config = value }) { }
        constexpr Packet(GetPinPowerResponse value) : tag(Type::GetPinPowerResponse), data(Data { .getPinPowerResponse = value }) { }
        constexpr Packet(GetPinModeResponse value) : tag(Type::GetPinModeResponse), data(Data { .getPinModeResponse = value }) { }
        constexpr Packet(SetPinPowerResponse value) : tag(Type::SetPinPowerResponse), data(Data { .setPinPowerResponse = value }) { }
        constexpr Packet(SetPinModeResponse value) : tag(Type::SetPinModeResponse), data(Data { .setPinModeResponse = value }) { }
        constexpr Packet(PinListen value) : tag(Type::PinListen), data(Data { .pinListen = value }) { }
        constexpr Packet(InvalidPinMode value) : tag(Type::InvalidPinMode), data(Data { .invalidPinMode = value }) { }
        constexpr Packet(InvalidPinId value) : tag(Type::InvalidPinId), data(Data { .invalidPinId = value }) { }
        constexpr Packet(InvalidPacketId value) : tag(Type::InvalidPacketId), data(Data { .invalidPacketId = value }) { }
        constexpr Packet(InvalidWriteToInput value) : tag(Type::InvalidWriteToInput), data(Data { .invalidWriteToInput = value }) { }
        constexpr Packet(InvalidUnsupportedMode value) : tag(Type::InvalidUnsupportedMode), data(Data { .invalidUnsupportedMode = value }) { }
        constexpr Packet(InvalidEscape value) : tag(Type::InvalidEscape), data(Data { .invalidEscape = value }) { }

        constexpr Packet(const Packet& original) :
            tag(original.tag),
            data(
                tag == Type::Pong ? Data { .pong = original.data.pong } :
                tag == Type::Config ? Data { .config = original.data.config } :
                tag == Type::GetPinPowerResponse ? Data { .getPinPowerResponse = original.data.getPinPowerResponse } :
                tag == Type::GetPinModeResponse ? Data { .getPinModeResponse = original.data.getPinModeResponse } :
                tag == Type::SetPinPowerResponse ? Data { .setPinPowerResponse = original.data.setPinPowerResponse } :
                tag == Type::SetPinModeResponse ? Data { .setPinModeResponse = original.data.setPinModeResponse } :
                tag == Type::PinListen ? Data { .pinListen = original.data.pinListen } :
                tag == Type::InvalidPinMode ? Data { .invalidPinMode = original.data.invalidPinMode } :
                tag == Type::InvalidPinId ? Data { .invalidPinId = original.data.invalidPinId } :
                tag == Type::InvalidPacketId ? Data { .invalidPacketId = original.data.invalidPacketId } :
                tag == Type::InvalidWriteToInput ? Data { .invalidWriteToInput = original.data.invalidWriteToInput } :
                tag == Type::InvalidUnsupportedMode ? Data { .invalidUnsupportedMode = original.data.invalidUnsupportedMode } :
                Data { .invalidEscape = original.data.invalidEscape }) { }

        ~Packet()
        {
            switch (tag)
            {
                case Type::Pong: data.pong.~Pong(); break;
                case Type::Config: data.config.~Config(); break;
                case Type::GetPinPowerResponse: data.getPinPowerResponse.~GetPinPowerResponse(); break;
                case Type::GetPinModeResponse: data.getPinModeResponse.~GetPinModeResponse(); break;
                case Type::SetPinPowerResponse: data.setPinPowerResponse.~SetPinPowerResponse(); break;
                case Type::SetPinModeResponse: data.setPinModeResponse.~SetPinModeResponse(); break;
                case Type::PinListen: data.pinListen.~PinListen(); break;
                case Type::InvalidPinMode: data.invalidPinMode.~InvalidPinMode(); break;
                case Type::InvalidPinId: data.invalidPinId.~InvalidPinId(); break;
                case Type::InvalidPacketId: data.invalidPacketId.~InvalidPacketId(); break;
                case Type::InvalidWriteToInput: data.invalidWriteToInput.~InvalidWriteToInput(); break;
                case Type::InvalidUnsupportedMode: data.invalidUnsupportedMode.~InvalidUnsupportedMode(); break;
                case Type::InvalidEscape: data.invalidEscape.~InvalidEscape(); break;
            }
        }

        [[nodiscard]] constexpr PacketType type() const
        {
            return tag;
        }

        [[nodiscard]] constexpr bool isPong() const
        {
            return tag == Type::Pong;
        }

        [[nodiscard]] Pong* pong()
        {
            return tag != Type::Pong ? nullptr : &data.pong;
        }

        [[nodiscard]] constexpr const Pong* pong() const
        {
            return tag != Type::Pong ? nullptr : &data.pong;
        }

        [[nodiscard]] constexpr bool isConfig() const
        {
            return tag == Type::Config;
        }

        [[nodiscard]] Config* config()
        {
            return tag != Type::Config ? nullptr : &data.config;
        }

        [[nodiscard]] constexpr const Config* config() const
        {
            return tag != Type::Config ? nullptr : &data.config;
        }

        [[nodiscard]] constexpr bool isGetPinPowerResponse() const
        {
            return tag == Type::GetPinPowerResponse;
        }

        [[nodiscard]] GetPinPowerResponse* getPinPowerResponse()
        {
            return tag != Type::GetPinPowerResponse ? nullptr : &data.getPinPowerResponse;
        }

        [[nodiscard]] constexpr const GetPinPowerResponse* getPinPowerResponse() const
        {
            return tag != Type::GetPinPowerResponse ? nullptr : &data.getPinPowerResponse;
        }

        [[nodiscard]] constexpr bool isGetPinModeResponse() const
        {
            return tag == Type::GetPinModeResponse;
        }

        [[nodiscard]] GetPinModeResponse* getPinModeResponse()
        {
            return tag != Type::GetPinModeResponse ? nullptr : &data.getPinModeResponse;
        }

        [[nodiscard]] constexpr const GetPinModeResponse* getPinModeResponse() const
        {
            return tag != Type::GetPinModeResponse ? nullptr : &data.getPinModeResponse;
        }

        [[nodiscard]] constexpr bool isSetPinPowerResponse() const
        {
            return tag == Type::SetPinPowerResponse;
        }

        [[nodiscard]] SetPinPowerResponse* setPinPowerResponse()
        {
            return tag != Type::SetPinPowerResponse ? nullptr : &data.setPinPowerResponse;
        }

        [[nodiscard]] constexpr const SetPinPowerResponse* setPinPowerResponse() const
        {
            return tag != Type::SetPinPowerResponse ? nullptr : &data.setPinPowerResponse;
        }

        [[nodiscard]] constexpr bool isSetPinModeResponse() const
        {
            return tag == Type::SetPinModeResponse;
        }

        [[nodiscard]] SetPinModeResponse* setPinModeResponse()
        {
            return tag != Type::SetPinModeResponse ? nullptr : &data.setPinModeResponse;
        }

        [[nodiscard]] constexpr const SetPinModeResponse* setPinModeResponse() const
        {
            return tag != Type::SetPinModeResponse ? nullptr : &data.setPinModeResponse;
        }

        [[nodiscard]] constexpr bool isPinListen() const
        {
            return tag == Type::PinListen;
        }

        [[nodiscard]] PinListen* pinListen()
        {
            return tag != Type::PinListen ? nullptr : &data.pinListen;
        }

        [[nodiscard]] constexpr const PinListen* pinListen() const
        {
            return tag != Type::PinListen ? nullptr : &data.pinListen;
        }

        [[nodiscard]] constexpr bool isInvalidPinMode() const
        {
            return tag == Type::InvalidPinMode;
        }

        [[nodiscard]] InvalidPinMode* invalidPinMode()
        {
            return tag != Type::InvalidPinMode ? nullptr : &data.invalidPinMode;
        }

        [[nodiscard]] constexpr const InvalidPinMode* invalidPinMode() const
        {
            return tag != Type::InvalidPinMode ? nullptr : &data.invalidPinMode;
        }

        [[nodiscard]] constexpr bool isInvalidPinId() const
        {
            return tag == Type::InvalidPinId;
        }

        [[nodiscard]] InvalidPinId* invalidPinId()
        {
            return tag != Type::InvalidPinId ? nullptr : &data.invalidPinId;
        }

        [[nodiscard]] constexpr const InvalidPinId* invalidPinId() const
        {
            return tag != Type::InvalidPinId ? nullptr : &data.invalidPinId;
        }

        [[nodiscard]] constexpr bool isInvalidPacketId() const
        {
            return tag == Type::InvalidPacketId;
        }

        [[nodiscard]] InvalidPacketId* invalidPacketId()
        {
            return tag != Type::InvalidPacketId ? nullptr : &data.invalidPacketId;
        }

        [[nodiscard]] constexpr const InvalidPacketId* invalidPacketId() const
        {
            return tag != Type::InvalidPacketId ? nullptr : &data.invalidPacketId;
        }

        [[nodiscard]] constexpr bool isInvalidWriteToInput() const
        {
            return tag == Type::InvalidWriteToInput;
        }

        [[nodiscard]] InvalidWriteToInput* invalidWriteToInput()
        {
            return tag != Type::InvalidWriteToInput ? nullptr : &data.invalidWriteToInput;
        }

        [[nodiscard]] constexpr const InvalidWriteToInput* invalidWriteToInput() const
        {
            return tag != Type::InvalidWriteToInput ? nullptr : &data.invalidWriteToInput;
        }

        [[nodiscard]] constexpr bool isInvalidUnsupportedMode() const
        {
            return tag == Type::InvalidUnsupportedMode;
        }

        [[nodiscard]] InvalidUnsupportedMode* invalidUnsupportedMode()
        {
            return tag != Type::InvalidUnsupportedMode ? nullptr : &data.invalidUnsupportedMode;
        }

        [[nodiscard]] constexpr const InvalidUnsupportedMode* invalidUnsupportedMode() const
        {
            return tag != Type::InvalidUnsupportedMode ? nullptr : &data.invalidUnsupportedMode;
        }

        [[nodiscard]] constexpr bool isInvalidEscape() const
        {
            return tag == Type::InvalidEscape;
        }

        [[nodiscard]] InvalidEscape* invalidEscape()
        {
            return tag != Type::InvalidEscape ? nullptr : &data.invalidEscape;
        }

        [[nodiscard]] constexpr const InvalidEscape* invalidEscape() const
        {
            return tag != Type::InvalidEscape ? nullptr : &data.invalidEscape;
        }
    };

    template<typename T, typename F>
    void serialize(F& sink, const T& value);

    template<typename F>
    void serialize(F& sink, const uint8_t& value)
    {
        sink(value);
    }

    template<typename F>
    void serialize(F& sink, const uint16_t& value)
    {
        uint8_t byte0 = value;
        uint8_t byte1 = value >> 8;
        sink(byte0);
        sink(byte1);
    }

    template<typename F>
    void serialize(F& sink, const PinMode& value)
    {
        serialize(sink, (uint8_t)value);
    }

    template<typename F>
    void serialize(F& sink, const Pong& value) { }

    template<typename F>
    void serialize(F& sink, const Config& value)
    {
        const int DIGITAL_INPUT = 1 << 0;
        const int DIGITAL_OUTPUT = 1 << 1;
        const int ANALOG_INPUT = 1 << 2;
        const int ANALOG_OUTPUT = 1 << 3;

        #ifdef BOARD_ARDUINO_UNO
        // version: short
        serialize(sink, (uint16_t)1);
        // name: string
        serialize(sink, (uint16_t)11);
        sink('a');
        sink('r');
        sink('d');
        sink('u');
        sink('i');
        sink('n');
        sink('o');
        sink('-');
        sink('u');
        sink('n');
        sink('o');
        // pins: Pin[]
        serialize(sink, (uint16_t)18);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)1);
        sink('2');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)1);
        sink('3');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)1);
        sink('4');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)1);
        sink('5');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)1);
        sink('6');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)1);
        sink('7');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)1);
        sink('8');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)1);
        sink('9');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)2);
        sink('1');
        sink('0');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)2);
        sink('1');
        sink('1');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)2);
        sink('1');
        sink('2');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)2);
        sink('1');
        sink('3');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)2);
        sink('A');
        sink('0');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_INPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)2);
        sink('A');
        sink('1');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_INPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)2);
        sink('A');
        sink('2');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_INPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)2);
        sink('A');
        sink('3');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_INPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)2);
        sink('A');
        sink('4');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_INPUT);
        // - Pin
        //   name: string
        serialize(sink, (uint16_t)2);
        sink('A');
        sink('5');
        //   flags: byte
        sink(DIGITAL_INPUT | DIGITAL_OUTPUT | ANALOG_INPUT);
        #else
        #error "Unknown board type"
        #endif
    }

    template<typename F>
    void serialize(F& sink, const GetPinModeResponse& value)
    {
        serialize(sink, value.pin);
        serialize(sink, value.mode);
    }

    template<typename F>
    void serialize(F& sink, const GetPinPowerResponse& value)
    {
        serialize(sink, value.pin);
        serialize(sink, value.power);
    }

    template<typename F>
    void serialize(F& sink, const SetPinPowerResponse& value)
    {
        serialize(sink, value.pin);
        serialize(sink, value.power);
    }

    template<typename F>
    void serialize(F& sink, const SetPinModeResponse& value)
    {
        serialize(sink, value.pin);
        serialize(sink, value.mode);
    }

    template<typename F>
    void serialize(F& sink, const PinListen& value)
    {
        serialize(sink, value.pin);
        serialize(sink, value.power);
    }

    template<typename F>
    void serialize(F& sink, const InvalidPinMode& value)
    {
        serialize(sink, value.byte);
    }

    template<typename F>
    void serialize(F& sink, const InvalidPinId& value)
    {
        serialize(sink, value.pin);
    }

    template<typename F>
    void serialize(F& sink, const InvalidPacketId& value)
    {
        serialize(sink, value.byte);
    }

    template<typename F>
    void serialize(F& sink, const InvalidWriteToInput& value)
    {
        serialize(sink, value.pin);
        serialize(sink, value.power);
    }

    template<typename F>
    void serialize(F& sink, const InvalidUnsupportedMode& value)
    {
        serialize(sink, value.pin);
        serialize(sink, value.mode);
    }

    template<typename F>
    void serialize(F& sink, const InvalidEscape& value)
    {
        serialize(sink, value.byte);
    }

    template<typename F>
    void serialize(F& sink, const Packet& value)
    {
        switch (value.type())
        {
            case PacketType::Pong:
                sink(0);
                serialize(sink, *value.pong());
                break;
            case PacketType::Config:
                sink(1);
                serialize(sink, *value.config());
                break;
            case PacketType::GetPinPowerResponse:
                sink(2);
                serialize(sink, *value.getPinPowerResponse());
                break;
            case PacketType::GetPinModeResponse:
                sink(3);
                serialize(sink, *value.getPinModeResponse());
                break;
            case PacketType::SetPinPowerResponse:
                sink(4);
                serialize(sink, *value.setPinPowerResponse());
                break;
            case PacketType::SetPinModeResponse:
                sink(5);
                serialize(sink, *value.setPinModeResponse());
                break;
            case PacketType::PinListen:
                sink(6);
                serialize(sink, *value.pinListen());
                break;
            case PacketType::InvalidPinMode:
                sink(101);
                serialize(sink, *value.invalidPinMode());
                break;
            case PacketType::InvalidPinId:
                sink(102);
                serialize(sink, *value.invalidPinId());
                break;
            case PacketType::InvalidPacketId:
                sink(103);
                serialize(sink, *value.invalidPacketId());
                break;
            case PacketType::InvalidWriteToInput:
                sink(104);
                serialize(sink, *value.invalidWriteToInput());
                break;
            case PacketType::InvalidUnsupportedMode:
                sink(105);
                serialize(sink, *value.invalidUnsupportedMode());
                break;
            case PacketType::InvalidEscape:
                sink(106);
                serialize(sink, *value.invalidUnsupportedMode());
                break;
        }
    }
}

#endif