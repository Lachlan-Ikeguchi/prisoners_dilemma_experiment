-- name: MEAN
-- defects with 75% probability

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.75 then
        return "defect"
    else
        return "cooperate"
    end
end
