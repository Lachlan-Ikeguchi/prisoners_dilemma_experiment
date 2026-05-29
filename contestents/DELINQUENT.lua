-- name: DELINQUENT
-- occasionally misbehaves

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.05 then
        return "defect"
    else
        return "cooperate"
    end
end
