
template<typename T, typename E>
struct Result
{
private:
    union Data
    {
        T ok;
        E err;
    };

    bool isErr;
    Data data;

public:
    Result(const T ok) 
    {
        this->isErr = false;
        this->data.ok = ok;
    }

    Result(const Result<T, E>& original) 
    {
        this->isErr = original.isErr;
        if (this->isErr)
            this->data.err = original.data.err;
        else
            this->data.ok = original.data.ok;
    }

    ~Result()
    {
        if (this->isErr)
            this->data.err.~E();
        else
            this->data.ok.~T();
    }

    [[nodiscard]] constexpr static Result<T, E> ok(T value)
    {
        Result<T, E> result;
        result.isErr = false;
        result.data.ok = value;
        return result;
    }

    [[nodiscard]] constexpr static Result<T, E> err(E value)
    {
        Result<T, E> result;
        result.isErr = true;
        result.data.err = value;
        return result;
    }

    [[nodiscard]] constexpr bool isOk() const
    {
        return !isErr;
    }

    [[nodiscard]] constexpr bool isErr() const
    {
        return isErr;
    }

    [[nodiscard]] constexpr Result<T, void> ok() const
    {
        if (!this->isErr)
            return Result<T, void>::ok(this->data.ok);
        else
            return Result<T, void>::err();
    }

    [[nodiscard]] constexpr Result<T, void> err() const
    {
        if (this->isErr)
            return Result<T, void>::ok(this->data.err);
        else
            return Result<T, void>::err();
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
    };

    bool isErr;
    Data data;

public:
    Result(const Result<T, void>& original) 
    {
        this->isErr = original.isErr;
        if (!this->isErr)
            this->data.ok = original.data.ok;
    }

    ~Result()
    {
        if (!this->isErr)
            this->data.ok.~T();
    }

    [[nodiscard]] constexpr static Result<T, void> ok(T value)
    {
        Result<T, void> result;
        result.isErr = false;
        result.data.ok = value;
        return result;
    }

    [[nodiscard]] constexpr static Result<void, void> err(void)
    {
        Result<void, void> result;
        result.isErr = true;
        return result;
    }

    [[nodiscard]] constexpr bool isOk() const
    {
        return !isErr;
    }

    [[nodiscard]] constexpr bool isErr() const
    {
        return isErr;
    }

    [[nodiscard]] constexpr Result<T, void> ok() const
    {
        if (!this->isErr)
            return Result<T, void>::ok(this->data.ok);
        else
            return Result<T, void>::err();
    }

    [[nodiscard]] constexpr Result<void, void> err() const
    {
        if (this->isErr)
            return Result<void, void>::ok();
        else
            return Result<void, void>::err();
    }

    constexpr T operator *() const
    {
        return this->data.ok;
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
    };

    bool isErr;
    Data data;

public:
    Result(const Result<void, E>& original) 
    {
        this->isErr = original.isErr;
        if (this->isErr)
            this->data.err = original.data.err;
    }

    ~Result()
    {
        if (this->isErr)
            this->data.err.~E();
    }

    [[nodiscard]] constexpr static Result<void, void> ok(void)
    {
        Result<void, void> result;
        result.isErr = false;
        return result;
    }

    [[nodiscard]] constexpr static Result<void, E> err(E value)
    {
        Result<void, E> result;
        result.isErr = true;
        result.data.err = value;
        return result;
    }

    [[nodiscard]] constexpr bool isOk() const
    {
        return !isErr;
    }

    [[nodiscard]] constexpr bool isErr() const
    {
        return isErr;
    }

    [[nodiscard]] constexpr Result<void, void> ok() const
    {
        if (!this->isErr)
            return Result<T, void>::ok();
        else
            return Result<T, void>::err();
    }

    [[nodiscard]] constexpr Result<E, void> err() const
    {
        if (this->isErr)
            return Result<T, void>::ok(this->data.err);
        else
            return Result<T, void>::err();
    }
};

template<typename T, typename E>
struct Result<void, void>
{
private:
    bool isErr;

public:
    Result(const Result<void, void>& original) 
    {
        this->isErr = original.isErr;
    }

    [[nodiscard]] constexpr static Result<void, void> ok(void)
    {
        Result<void, void> result;
        result.isErr = false;
        return result;
    }

    [[nodiscard]] constexpr static Result<void, void> err(void)
    {
        Result<void, void> result;
        result.isErr = true;
        return result;
    }

    [[nodiscard]] constexpr bool isOk() const
    {
        return !isErr;
    }

    [[nodiscard]] constexpr bool isErr() const
    {
        return isErr;
    }

    [[nodiscard]] constexpr Result<void, void> ok() const
    {
        if (!this->isErr)
            return Result<T, void>::ok();
        else
            return Result<T, void>::err();
    }

    [[nodiscard]] constexpr Result<void, void> err() const
    {
        if (this->isErr)
            return Result<T, void>::ok();
        else
            return Result<T, void>::err();
    }
};