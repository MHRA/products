if redis.call("GET", KEYS[1]) == "Accepted" or redis.call("EXISTS", KEYS[1]) == 0 then
    return redis.call("SET", KEYS[1], ARGV[1])
else
    return nil
end
