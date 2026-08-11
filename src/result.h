#ifndef result_h
#define result_h

#include <new>
#include "result.h"

template<typename T, typename E>
struct Result
{
private:
    union Data
    {
        T ok;
        E err;

        constexpr Data() { }
        ~Data() { }
    };

    bool tag;
    Data data;

private:
    Result(E err, int) : tag(true)
    {
        ::new(&data.err) E(err);
    }

public:
    Result(T ok) : tag(false)
    {
        ::new(&data.ok) T(ok);
    }

    Result(const Result<T, E>& original) : tag(original.tag)
    {
        if (tag)
            ::new(&data.err) E(original.data.err);
        else
            ::new(&data.ok) T(original.data.ok);
    }

    ~Result()
    {
        if (tag)
            data.err.~E();
        else
            data.ok.~T();
    }

    Result<T, E>& operator=(const Result<T, E>& other)
    {
        if (this != &other)
        {
            if (tag)
            {
                if (other.tag)
                {
                    data.err = other.data.err;
                }
                else
                {
                    tag = false;
                    data.err.~E();
                    ::new(&data.ok) T(other.data.ok);
                }
            }
            else
            {
                if (other.tag)
                {
                    tag = true;
                    data.ok.~T();
                    ::new(&data.err) E(other.data.err);
                }
                else
                {
                    data.ok = other.data.ok;
                }
            }
        }
        return *this;
    }

    static constexpr Result<T, E> makeOk(T value)
    {
        return Result<T, E>(value);
    }

    static constexpr Result<T, E> makeErr(E value)
    {
        return Result<T, E>(value, 0);
    }

public:
    [[nodiscard]] constexpr bool isOk() const
    {
        return !tag;
    }

    [[nodiscard]] constexpr bool isErr() const
    {
        return tag;
    }

    [[nodiscard]] T* ok()
    {
        return tag ? nullptr : &data.ok;
    }

    [[nodiscard]] constexpr const T* ok() const
    {
        return tag ? nullptr : &data.ok;
    }

    [[nodiscard]] E* err()
    {
        return tag ? &data.err : nullptr;
    }

    [[nodiscard]] constexpr const E* err() const
    {
        return tag ? &data.err : nullptr;
    }

    [[nodiscard]] T& okOr(T& other)
    {
        return tag ? other : data.ok;
    }

    [[nodiscard]] const T& okOr(const T& other) const
    {
        return tag ? other : data.ok;
    }

    [[nodiscard]] E& errOr(E& other)
    {
        return tag ? data.err : other;
    }

    [[nodiscard]] const E& errOr(const E& other) const
    {
        return tag ? data.err : other;
    }
};

template<>
struct Result<void, void>
{
private:
    bool tag;

private:
    constexpr Result(int) : tag(true) { }

public:
    constexpr Result() : tag(false) { }

    constexpr Result(const Result<void, void>& original) : tag(original.tag) { }

    Result<void, void>& operator=(const Result<void, void>& other)
    {
        if (this != &other) { tag = other.tag; }
        return *this;
    }

    static constexpr Result<void, void> makeOk()
    {
        return Result<void, void>();
    }

    static constexpr Result<void, void> makeErr()
    {
        return Result<void, void>(0);
    }

public:
    [[nodiscard]] constexpr bool isOk() const
    {
        return !tag;
    }

    [[nodiscard]] constexpr bool isErr() const
    {
        return tag;
    }

    [[nodiscard]] constexpr bool ok() const
    {
        return !tag;
    }

    [[nodiscard]] constexpr bool err() const
    {
        return tag;
    }
};

template<typename T>
struct Result<T, void>
{
private:
    union Data
    {
        T ok;
        char err;

        constexpr Data() { }
        ~Data() { }
    };

    bool tag;
    Data data;

private:
    constexpr Result(char, int) : tag(true) { }

public:
    Result(T ok) : tag(false)
    {
        ::new(&data.ok) T(ok);
    }

    Result(const Result<T, void>& original) : tag(original.tag)
    {
        if (!tag)
            ::new(&data.ok) T(original.data.ok);
    }

    ~Result()
    {
        if (!tag)
            data.ok.~T();
    }

    Result<T, void>& operator=(const Result<T, void>& other)
    {
        if (this != &other)
        {
            if (tag)
            {
                if (!other.tag)
                {
                    tag = false;
                    ::new(&data.ok) T(other.data.ok);
                }
            }
            else
            {
                if (other.tag)
                {
                    tag = true;
                    data.ok.~T();
                }
                else
                {
                    data.ok = other.data.ok;
                }
            }
        }
        return *this;
    }

    static constexpr Result<T, void> makeOk(T value)
    {
        return Result<T, void>(value);
    }

    static constexpr Result<T, void> makeErr()
    {
        return Result<T, void>(0, 0);
    }

public:
    [[nodiscard]] constexpr bool isOk() const
    {
        return !tag;
    }

    [[nodiscard]] constexpr bool isErr() const
    {
        return tag;
    }

    [[nodiscard]] T* ok()
    {
        return tag ? nullptr : &data.ok;
    }

    [[nodiscard]] constexpr const T* ok() const
    {
        return tag ? nullptr : &data.ok;
    }

    [[nodiscard]] constexpr bool err() const
    {
        return tag;
    }

    [[nodiscard]] T& okOr(T& other)
    {
        return tag ? other : data.ok;
    }

    [[nodiscard]] const T& okOr(const T& other) const
    {
        return tag ? other : data.ok;
    }

    [[nodiscard]] constexpr T& operator*() const
    {
        return data.ok;
    }

    [[nodiscard]] constexpr T* operator->() const
    {
        return &data.ok;
    }
};

template<typename E>
struct Result<void, E>
{
private:
    union Data
    {
        char ok;
        E err;

        constexpr Data() { }
        ~Data() { }
    };

    bool tag;
    Data data;

private:
    Result(E err, int) : tag(true)
    {
        ::new(&data.err) E(err);
    }

public:
    constexpr Result() : tag(false) { }

    Result(const Result<void, E>& original) : tag(original.tag)
    {
        if (tag)
            ::new(&data.err) E(original.data.err);
    }

    ~Result()
    {
        if (tag)
            data.err.~E();
    }

    Result<void, E>& operator=(const Result<void, E>& other)
    {
        if (this != &other)
        {
            if (tag)
            {
                if (other.tag)
                {
                    data.err = other.data.err;
                }
                else
                {
                    tag = false;
                    data.err.~E();
                }
            }
            else
            {
                if (other.tag)
                {
                    tag = true;
                    ::new(&data.err) E(other.data.err);
                }
            }
        }
        return *this;
    }

    static constexpr Result<void, E> makeOk()
    {
        return Result<void, E>();
    }

    static constexpr Result<void, E> makeErr(E value)
    {
        return Result<void, E>(value, 0);
    }

public:
    [[nodiscard]] constexpr bool isOk() const
    {
        return !tag;
    }

    [[nodiscard]] constexpr bool isErr() const
    {
        return tag;
    }

    [[nodiscard]] constexpr bool ok() const
    {
        return !tag;
    }

    [[nodiscard]] E* err()
    {
        return tag ? &data.err : nullptr;
    }

    [[nodiscard]] constexpr const E* err() const
    {
        return tag ? &data.err : nullptr;
    }

    [[nodiscard]] E& errOr(E& other)
    {
        return tag ? data.err : other;
    }

    [[nodiscard]] const E& errOr(const E& other) const
    {
        return tag ? data.err : other;
    }
};

#endif