-- name: BULLY
-- defects if opponent cooperated last round, otherwise cooperates

function decide(round, history)
    if round == 1 then
        return "defect"
    end
    
    if history[#history] == "cooperate" then
        return "defect"
    else
        return "cooperate"
    end
end
