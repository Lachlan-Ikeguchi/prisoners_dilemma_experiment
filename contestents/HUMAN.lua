-- name: HUMAN
-- simulates human-like behavior: mostly cooperates but occasionally defects

math.randomseed(os.time())

function decide(round, history)
    -- 85% chance to cooperate
    if math.random() < 0.85 then
        return "cooperate"
    else
        return "defect"
    end
end
