-- name: FINAL JUDGE
-- bases decision on entire history, with forgiveness for early defections

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    local early_defects = 0
    local late_defects = 0
    local midpoint = math.floor(#history / 2)
    
    for i = 1, midpoint do
        if history[i] == "defect" then early_defects = early_defects + 1 end
    end
    for i = midpoint + 1, #history do
        if history[i] == "defect" then late_defects = late_defects + 1 end
    end
    
    -- Forgive early defections, punish recent ones
    if late_defects > 0 then
        return "defect"
    else
        return "cooperate"
    end
end
