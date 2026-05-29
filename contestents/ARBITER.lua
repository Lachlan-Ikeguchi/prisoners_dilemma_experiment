-- name: ARBITER
-- judges based on if opponent has defected more than cooperated

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    local coop = 0
    local defect = 0
    for i = 1, #history do
        if history[i] == "cooperate" then coop = coop + 1
        else defect = defect + 1 end
    end
    
    if defect > coop then
        return "defect"
    else
        return "cooperate"
    end
end
