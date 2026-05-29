-- name: RISK TAKER
-- defects with 60% probability

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.60 then
        return "defect"
    else
        return "cooperate"
    end
end
