#ifndef incoming_h
#define incoming_h

#include <stdint.h>
#include "pin.h"

namespace incoming
{
    enum struct PacketType
    {
        Ping,
        WholeConfig,
        GetPinPower,
        GetPinMode,
        SetPinPower,
        SetPinMode,
    };

    struct Ping
    {
        [[nodiscard]] static constexpr PacketType type() { return PacketType::Ping; }
    };

    struct WholeConfig
    {
        [[nodiscard]] static constexpr PacketType type() { return PacketType::WholeConfig; }
    };

    struct GetPinPower
    {
        uint8_t pin;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::GetPinPower; }
    };

    struct GetPinMode
    {
        uint8_t pin;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::GetPinMode; }
    };

    struct SetPinPower
    {
        uint8_t pin;
        uint8_t power;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::SetPinPower; }
    };

    struct SetPinMode
    {
        uint8_t pin;
        PinMode mode;

        [[nodiscard]] static constexpr PacketType type() { return PacketType::SetPinMode; }
    };

    struct Packet
    {
    private:
        typedef PacketType Type;

        union Data
        {
            Ping ping;
            WholeConfig wholeConfig;
            GetPinPower getPinPower;
            GetPinMode getPinMode;
            SetPinPower setPinPower;
            SetPinMode setPinMode;
        };

        Type tag;
        Data data;

    public:
        constexpr Packet(Ping value) : tag(Type::Ping), data(Data { .ping = value }) { }
        constexpr Packet(WholeConfig value) : tag(Type::WholeConfig), data(Data { .wholeConfig = value }) { }
        constexpr Packet(GetPinPower value) : tag(Type::GetPinPower), data(Data { .getPinPower = value }) { }
        constexpr Packet(GetPinMode value) : tag(Type::GetPinMode), data(Data { .getPinMode = value }) { }
        constexpr Packet(SetPinPower value) : tag(Type::SetPinPower), data(Data { .setPinPower = value }) { }
        constexpr Packet(SetPinMode value) : tag(Type::SetPinMode), data(Data { .setPinMode = value }) { }

        constexpr Packet(const Packet& original) :
            tag(original.tag),
            data(
                tag == Type::Ping ? Data { .ping = original.data.ping } :
                tag == Type::WholeConfig ? Data { .wholeConfig = original.data.wholeConfig } :
                tag == Type::GetPinPower ? Data { .getPinPower = original.data.getPinPower } :
                tag == Type::GetPinMode ? Data { .getPinMode = original.data.getPinMode } :
                tag == Type::SetPinPower ? Data { .setPinPower = original.data.setPinPower } :
                Data { .setPinMode = original.data.setPinMode }) { }

        ~Packet()
        {
            switch (tag)
            {
                case Type::Ping: data.ping.~Ping(); break;
                case Type::WholeConfig: data.wholeConfig.~WholeConfig(); break;
                case Type::GetPinPower: data.getPinPower.~GetPinPower(); break;
                case Type::GetPinMode: data.getPinMode.~GetPinMode(); break;
                case Type::SetPinPower: data.setPinPower.~SetPinPower(); break;
                case Type::SetPinMode: data.setPinMode.~SetPinMode(); break;
            }
        }

        [[nodiscard]] constexpr PacketType type() const
        {
            return tag;
        }

        [[nodiscard]] constexpr bool isPing() const
        {
            return tag == Type::Ping;
        }

        [[nodiscard]] Ping* ping()
        {
            return tag != Type::Ping ? nullptr : &data.ping;
        }

        [[nodiscard]] constexpr const Ping* ping() const
        {
            return tag != Type::Ping ? nullptr : &data.ping;
        }

        [[nodiscard]] constexpr bool isWholeConfig() const
        {
            return tag == Type::WholeConfig;
        }

        [[nodiscard]] WholeConfig* wholeConfig()
        {
            return tag != Type::WholeConfig ? nullptr : &data.wholeConfig;
        }

        [[nodiscard]] constexpr const WholeConfig* wholeConfig() const
        {
            return tag != Type::WholeConfig ? nullptr : &data.wholeConfig;
        }

        [[nodiscard]] constexpr bool isGetPinPower() const
        {
            return tag == Type::GetPinPower;
        }

        [[nodiscard]] GetPinPower* getPinPower()
        {
            return tag != Type::GetPinPower ? nullptr : &data.getPinPower;
        }

        [[nodiscard]] constexpr const GetPinPower* getPinPower() const
        {
            return tag != Type::GetPinPower ? nullptr : &data.getPinPower;
        }

        [[nodiscard]] constexpr bool isGetPinMode() const
        {
            return tag == Type::GetPinMode;
        }

        [[nodiscard]] GetPinMode* getPinMode()
        {
            return tag != Type::GetPinMode ? nullptr : &data.getPinMode;
        }

        [[nodiscard]] constexpr const GetPinMode* getPinMode() const
        {
            return tag != Type::GetPinMode ? nullptr : &data.getPinMode;
        }

        [[nodiscard]] constexpr bool isSetPinPower() const
        {
            return tag == Type::SetPinPower;
        }

        [[nodiscard]] SetPinPower* setPinPower()
        {
            return tag != Type::SetPinPower ? nullptr : &data.setPinPower;
        }

        [[nodiscard]] constexpr const SetPinPower* setPinPower() const
        {
            return tag != Type::SetPinPower ? nullptr : &data.setPinPower;
        }

        [[nodiscard]] constexpr bool isSetPinMode() const
        {
            return tag == Type::SetPinMode;
        }

        [[nodiscard]] SetPinMode* setPinMode()
        {
            return tag != Type::SetPinMode ? nullptr : &data.setPinMode;
        }

        [[nodiscard]] constexpr const SetPinMode* setPinMode() const
        {
            return tag != Type::SetPinMode ? nullptr : &data.setPinMode;
        }
    };

    template<typename E, typename F>
    Result<uint8_t, E> deserializeUint8(F& source)
    {
        return source();
    }

    template<typename E, typename F>
    Result<uint16_t, E> deserializeUint16(F& source)
    {
        Result<uint8_t, E> byte0 = source();
        if (byte0.isErr())
            return Result<uint16_t, E>::makeErr(*byte0.err());
        Result<uint8_t, E> byte1 = source();
        if (byte1.isErr())
            return Result<uint16_t, E>::makeErr(*byte1.err());
        return *byte0.ok() | (*byte1.ok() << 8);
    }

    template<typename E, typename F>
    Result<Ping, E> deserializePing(F& source)
    {
        return Ping { };
    }

    template<typename E, typename F>
    Result<WholeConfig, E> deserializeWholeConfig(F& source)
    {
        return WholeConfig { };
    }

    template<typename E, typename F>
    Result<GetPinPower, E> deserializeGetPingPower(F& source)
    {
        Result<uint8_t, E> pin = deserializeUint8<E, F>(source);
        if (pin.isErr())
            return Result<GetPinPower, E>::makeErr(*pin.err());
        return GetPinPower { *pin.ok() };
    }

    template<typename E, typename F>
    Result<GetPinMode, E> deserializeGetPinMode(F& source)
    {
        Result<uint8_t, E> pin = deserializeUint8<E, F>(source);
        if (pin.isErr())
            return Result<GetPinMode, E>::makeErr(*pin.err());
        return GetPinMode { *pin.ok() };
    }

    template<typename E, typename F>
    Result<SetPinPower, E> deserializeSetPinPower(F& source)
    {
        Result<uint8_t, E> pin = deserializeUint8<E, F>(source);
        if (pin.isErr())
            return Result<SetPinPower, E>::makeErr(*pin.err());
        Result<uint8_t, E> power = deserializeUint8<E, F>(source);
        if (power.isErr())
            return Result<SetPinPower, E>::makeErr(*power.err());
        return SetPinPower { *pin.ok(), *power.ok() };
    }

    template<typename E, typename F>
    Result<SetPinMode, E> deserializeSetPinMode(F& source)
    {
        Result<uint8_t, E> pin = deserializeUint8<E, F>(source);
        if (pin.isErr())
            return Result<SetPinMode, E>::makeErr(*pin.err());
        Result<uint8_t, E> mode = deserializeUint8<E, F>(source);
        if (mode.isErr())
            return Result<SetPinMode, E>::makeErr(*mode.err());
        PinMode modeResult;
        switch (*mode.ok())
        {
            case 0: modeResult = PinMode::DigitalInput; break;
            case 1: modeResult = PinMode::DigitalListen; break;
            case 2: modeResult = PinMode::DigitalOutput; break;
            case 3: modeResult = PinMode::AnalogInput; break;
            case 4: modeResult = PinMode::AnalogOutput; break;
            default:
                return Result<SetPinMode, E>::makeErr(E::invalidPinMode());
        }
        return SetPinMode { *pin.ok(), modeResult };
    }

    template<typename E, typename F>
    Result<Packet, E> deserializePacket(F& source)
    {
        Result<uint8_t, E> type = deserializeUint8<E, F>(source);
        if (type.isErr())
            return Result<Packet, E>::makeErr(*type.err());
        switch (*type.ok())
        {
            case 0:
            {
                Result<Ping, E> value = deserializePing<E, F>(source);
                if (value.isErr())
                    return Result<Packet, E>::makeErr(*value.err());
                return Packet { *value.ok() };
            }
            case 1:
            {
                Result<WholeConfig, E> value = deserializeWholeConfig<E, F>(source);
                if (value.isErr())
                    return Result<Packet, E>::makeErr(*value.err());
                return Packet { *value.ok() };
            }
            case 2:
            {
                Result<GetPinPower, E> value = deserializeGetPingPower<E, F>(source);
                if (value.isErr())
                    return Result<Packet, E>::makeErr(*value.err());
                return Packet { *value.ok() };
            }
            case 3:
            {
                Result<GetPinMode, E> value = deserializeGetPinMode<E, F>(source);
                if (value.isErr())
                    return Result<Packet, E>::makeErr(*value.err());
                return Packet { *value.ok() };
            }
            case 4:
            {
                Result<SetPinPower, E> value = deserializeSetPinPower<E, F>(source);
                if (value.isErr())
                    return Result<Packet, E>::makeErr(*value.err());
                return Packet { *value.ok() };
            }
            case 5:
            {
                Result<SetPinMode, E> value = deserializeSetPinMode<E, F>(source);
                if (value.isErr())
                    return Result<Packet, E>::makeErr(*value.err());
                return Packet { *value.ok() };
            }
            default:
                return Result<Packet, E>::makeErr(E::invalidPacketId());
        }
    }
}

#endif