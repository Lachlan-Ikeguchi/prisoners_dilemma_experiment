-- name: BENEVOLENT
-- cooperates with 80% probability

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.8 then
        return "cooperate"
    else
        return "defect"
    end
end
