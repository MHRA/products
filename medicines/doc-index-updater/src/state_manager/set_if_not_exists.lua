if redis.call("EXISTS", KEYS[1]) == 0 then
    redis.call("SET", KEYS[1], ARGV[1])
end

return redis.call("GET", KEYS[1])
