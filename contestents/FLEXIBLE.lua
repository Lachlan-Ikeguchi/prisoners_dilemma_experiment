-- name: FLEXIBLE
-- adapts forgiveness threshold based on game progress

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    -- Forgiveness decreases as game progresses
    local forgiveness = math.max(1, 3 - math.floor(round / 30))
    
    local recent_defects = 0
    local start = math.max(1, #history - forgiveness + 1)
    for i = start, #history do
        if history[i] == "defect" then recent_defects = recent_defects + 1 end
    end
    
    if recent_defects >= forgiveness then
        return "defect"
    else
        return "cooperate"
    end
end
