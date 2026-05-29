-- name: BETRAYER
-- defects with 80% probability

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.8 then
        return "defect"
    else
        return "cooperate"
    end
end
