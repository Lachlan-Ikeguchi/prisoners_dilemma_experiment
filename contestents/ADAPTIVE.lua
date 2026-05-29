-- name: ADAPTIVE
-- adjusts strategy based on opponent's cooperation rate, with forgiveness

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    local coop_count = 0
    for i = 1, #history do
        if history[i] == "cooperate" then coop_count = coop_count + 1 end
    end
    
    local coop_rate = coop_count / #history
    
    -- If opponent cooperates >60%, cooperate; if <40%, defect; else random
    if coop_rate > 0.6 then
        return "cooperate"
    elseif coop_rate < 0.4 then
        return "defect"
    else
        -- Borderline: mirror last move
        return history[#history]
    end
end
