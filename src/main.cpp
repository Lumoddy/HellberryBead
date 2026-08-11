#include <Arduino.h>
#include "result.h"

template<typename T, typename E, typename F>
Result<T, E> deserialize(F source);

template<typename E, typename F>
Result<uint8_t, E> deserialize(F source)
{
    Result<uint8_t, E> byte = source();
    return byte;
}

template<typename E, typename F>
Result<uint16_t, E> deserialize(F source)
{
    Result<uint8_t, E> byte0 = source();
    if (byte0.isErr())
        return Result<uint16_t, E>::err(*byte0.err());
    Result<uint8_t, E> byte1 = source();
    if (byte1.isErr())
        return Result<uint16_t, E>::err(*byte1.err());
    return Result<uint16_t, E>::ok(*byte0.ok() | (*byte1.ok() << 8));
}

template<typename E, typename F>
Result<uint16_t, E> deserialize(F source)
{
    Result<uint8_t, E> byte0 = source();
    if (byte0.isErr())
        return Result<uint16_t, E>::err(*byte0.err());
    Result<uint8_t, E> byte1 = source();
    if (byte1.isErr())
        return Result<uint16_t, E>::err(*byte1.err());
    return Result<uint16_t, E>::ok(*byte0.ok() | (*byte1.ok() << 8));
}

void setup()
{
    
}

void loop()
{
    Result<int, int> x = Result<int, int>::ok(1);

    // put your main code here, to run repeatedly:
}