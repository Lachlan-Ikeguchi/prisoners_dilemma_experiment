-- name: GRADUAL
-- starts forgiving, becomes stricter over time

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    -- Forgiveness threshold decreases over time
    local threshold = math.max(0.1, 1.0 - (round / 100.0))
    
    local defect_count = 0
    for i = 1, #history do
        if history[i] == "defect" then
            defect_count = defect_count + 1
        end
    end
    
    local defect_ratio = defect_count / #history
    
    if defect_ratio > threshold then
        return "defect"
    else
        return "cooperate"
    end
end
