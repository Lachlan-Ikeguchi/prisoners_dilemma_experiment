-- name: BARKER
-- defects on prime-numbered rounds

function is_prime(n)
    if n < 2 then return false end
    for i = 2, math.sqrt(n) do
        if n % i == 0 then return false end
    end
    return true
end

function decide(round, history)
    if is_prime(round) then
        return "defect"
    else
        return "cooperate"
    end
end
