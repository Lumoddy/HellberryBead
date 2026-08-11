#ifndef state_h
#define state_h

/// Digital-In, Digital-Out
struct DiDoState
{
private:
    uint8_t pin;
    uint8_t mode;
    bool wasHigh;

public:
    constexpr DiDoState(uint8_t pin): pin(pin), mode(PinMode::DigitalInput), wasHigh(false) { }

public:
    void begin()
    {
        pinMode(pin, INPUT_PULLUP);
    }

    uint16_t getPower() const
    {
        switch (mode)
        {
            case PinMode::DigitalInput: return digitalRead(pin);
            case PinMode::DigitalListen: return digitalRead(pin);
            case PinMode::DigitalOutput: return digitalRead(pin);
        }

        return 0;
    }

    Result<uint16_t, void> pollListen()
    {
        if (mode == PinMode::DigitalListen)
        {
            bool isHigh = digitalRead(pin);
            if (wasHigh != isHigh)
            {
                wasHigh = isHigh;
                return (uint16_t)isHigh;
            }
        }

        return Result<uint16_t, void>::makeErr();
    }

    PinMode getMode() const
    {
        switch (mode)
        {
            case PinMode::DigitalInput: return PinMode::DigitalInput;
            case PinMode::DigitalListen: return PinMode::DigitalListen;
            case PinMode::DigitalOutput: return PinMode::DigitalOutput;
        }

        return (PinMode)0;
    }

    Result<void, outgoing::Packet> setPower(uint8_t power)
    {
        switch (mode)
        {
            case PinMode::DigitalOutput: digitalWrite(pin, power);
            default:
                return Result<void, outgoing::Packet>::makeErr(outgoing::InvalidWriteToInput { });
        }

        return Result<void, outgoing::Packet>();
    }

    Result<void, outgoing::Packet> setMode(PinMode newMode)
    {
        switch (newMode)
        {
            case PinMode::DigitalInput:
                pinMode(pin, INPUT_PULLUP);
                mode = PinMode::DigitalInput;
                break;
            case PinMode::DigitalListen:
                pinMode(pin, INPUT_PULLUP);
                mode = PinMode::DigitalListen;
                break;
            case PinMode::DigitalOutput:
                pinMode(pin, OUTPUT);
                mode = PinMode::DigitalOutput;
                break;
            default:
                return Result<void, outgoing::Packet>::makeErr(outgoing::InvalidUnsupportedMode { });
        }

        return Result<void, outgoing::Packet>::makeOk();
    }
};

/// Digital-In, Digital-Out, Analog-In
struct DiDoAiState
{
private:
    uint8_t pin;
    uint8_t mode;

public:
    constexpr DiDoAiState(uint8_t pin): pin(pin), mode(PinMode::DigitalInput) { }

public:
    void begin()
    {
        pinMode(pin, INPUT_PULLUP);
    }

    uint16_t getPower() const
    {
        switch (mode)
        {
            case PinMode::DigitalInput: return digitalRead(pin);
            case PinMode::DigitalListen: return digitalRead(pin);
            case PinMode::DigitalOutput: return digitalRead(pin);
            case PinMode::AnalogInput: return analogRead(pin);
        }

        return 0;
    }

    Result<uint16_t, void> pollListen() const
    {
        return Result<uint16_t, void>::makeErr();
    }

    PinMode getMode() const
    {
        switch (mode)
        {
            case PinMode::DigitalInput: return PinMode::DigitalInput;
            case PinMode::DigitalListen: return PinMode::DigitalListen;
            case PinMode::DigitalOutput: return PinMode::DigitalOutput;
            case PinMode::AnalogInput: return PinMode::AnalogInput;
        }

        return (PinMode)0;
    }

    Result<void, outgoing::Packet> setPower(uint8_t power) const
    {
        switch (mode)
        {
            case PinMode::DigitalOutput: digitalWrite(pin, power);
            default:
                return Result<void, outgoing::Packet>::makeErr(outgoing::InvalidWriteToInput { });
        }

        return Result<void, outgoing::Packet>();
    }

    Result<void, outgoing::Packet> setMode(PinMode newMode)
    {
        switch (newMode)
        {
            case PinMode::DigitalInput:
                pinMode(pin, INPUT_PULLUP);
                mode = PinMode::DigitalInput;
                break;
            case PinMode::DigitalListen:
                pinMode(pin, INPUT_PULLUP);
                mode = PinMode::DigitalListen;
                break;
            case PinMode::DigitalOutput:
                pinMode(pin, OUTPUT);
                mode = PinMode::DigitalOutput;
                break;
            case PinMode::AnalogInput:
                pinMode(pin, INPUT);
                mode = PinMode::AnalogInput;
                break;
            default:
                return Result<void, outgoing::Packet>::makeErr(outgoing::InvalidUnsupportedMode { });
        }

        return Result<void, outgoing::Packet>::makeOk();
    }
};

/// Digital-In, Digital-Out, Analog-Out
struct DiDoAoState
{
private:
    uint8_t pin;
    uint8_t mode;

public:
    constexpr DiDoAoState(uint8_t pin): pin(pin), mode(PinMode::DigitalInput) { }

public:
    void begin()
    {
        pinMode(pin, INPUT_PULLUP);
    }

    uint16_t getPower() const
    {
        switch (mode)
        {
            case PinMode::DigitalInput: return digitalRead(pin);
            case PinMode::DigitalListen: return digitalRead(pin);
            case PinMode::DigitalOutput: return digitalRead(pin);
            case PinMode::AnalogOutput: return analogRead(pin);
        }

        return 0;
    }

    Result<uint16_t, void> pollListen() const
    {
        return Result<uint16_t, void>::makeErr();
    }

    PinMode getMode() const
    {
        switch (mode)
        {
            case PinMode::DigitalInput: return PinMode::DigitalInput;
            case PinMode::DigitalListen: return PinMode::DigitalListen;
            case PinMode::DigitalOutput: return PinMode::DigitalOutput;
            case PinMode::AnalogOutput: return PinMode::AnalogOutput;
        }

        return (PinMode)0;
    }

    Result<void, outgoing::Packet> setPower(uint8_t power) const
    {
        switch (mode)
        {
            case PinMode::DigitalOutput: digitalWrite(pin, power);
            case PinMode::AnalogOutput: analogWrite(pin, power);
            default:
                return Result<void, outgoing::Packet>::makeErr(outgoing::InvalidWriteToInput { });
        }

        return Result<void, outgoing::Packet>();
    }

    Result<void, outgoing::Packet> setMode(PinMode newMode)
    {
        switch (newMode)
        {
            case PinMode::DigitalInput:
                pinMode(pin, INPUT_PULLUP);
                mode = PinMode::DigitalInput;
                break;
            case PinMode::DigitalListen:
                pinMode(pin, INPUT_PULLUP);
                mode = PinMode::DigitalListen;
                break;
            case PinMode::DigitalOutput:
                pinMode(pin, OUTPUT);
                mode = PinMode::DigitalOutput;
                break;
            case PinMode::AnalogOutput:
                pinMode(pin, OUTPUT);
                mode = PinMode::AnalogOutput;
                break;
            default:
                return Result<void, outgoing::Packet>::makeErr(outgoing::InvalidUnsupportedMode { });
        }

        return Result<void, outgoing::Packet>::makeOk();
    }
};

#endif