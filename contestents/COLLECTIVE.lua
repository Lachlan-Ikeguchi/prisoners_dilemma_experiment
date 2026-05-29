-- name: COLLECTIVE
-- tries to achieve mutual cooperation by starting with defect then cooperating

function decide(round, history)
    if round == 1 then
        return "defect"
    end
    
    if round == 2 then
        return "cooperate"
    end
    
    -- If opponent cooperated on round 2, keep cooperating
    if #history >= 2 and history[2] == "cooperate" then
        return "cooperate"
    end
    
    return "defect"
end
