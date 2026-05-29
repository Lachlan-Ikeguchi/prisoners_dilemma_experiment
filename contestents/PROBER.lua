-- name: PROBER
-- starts with defect, then cooperates, testing opponent's response

function decide(round, history)
    if round == 1 then
        return "defect"
    end
    
    if round == 2 then
        return "cooperate"
    end
    
    if #history >= 1 then
        -- If opponent cooperated after our defection, they're nice
        if history[1] == "cooperate" then
            return "cooperate"
        end
    end
    
    -- If they defected, keep defecting
    return "defect"
end
