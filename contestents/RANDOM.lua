-- name: RANDOM
-- random chance to cooperate or defect

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.5 then
        return "cooperate"
    else
        return "defect"
    end
end
