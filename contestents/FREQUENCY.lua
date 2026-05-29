-- name: FREQUENCY
-- uses opponent's move frequency from the entire history

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    local coop = 0
    for i = 1, #history do
        if history[i] == "cooperate" then coop = coop + 1 end
    end
    
    local coop_rate = coop / #history
    
    -- Threshold increases over time
    local threshold = 0.5 + (round / 200.0)
    
    if coop_rate > threshold then
        return "cooperate"
    else
        return "defect"
    end
end
