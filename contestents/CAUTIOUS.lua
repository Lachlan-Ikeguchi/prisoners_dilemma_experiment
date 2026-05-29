-- name: CAUTIOUS
-- defects first, then cooperates if opponent doesn't retaliate

function decide(round, history)
    if round == 1 then return "defect" end
    if round == 2 then return "cooperate" end
    
    if #history >= 1 and history[1] == "cooperate" then
        return "cooperate"
    else
        return "defect"
    end
end
