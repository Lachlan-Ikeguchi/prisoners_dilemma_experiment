-- name: MOSTLY COOPERATE
-- cooperates with 90% probability

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.90 then
        return "cooperate"
    else
        return "defect"
    end
end
