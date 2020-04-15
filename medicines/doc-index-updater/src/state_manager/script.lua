if redis.call("EXISTS", KEYS[1]) == 0 then
    return redis.call("SET", KEYS[1], "Accepted")
else
    return nil
end