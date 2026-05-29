-- name: CRAFTY
-- complex deception strategy

function decide(round, history)
    if round <= 2 then return "cooperate" end
    if round == 3 then return "defect" end
    
    if #history >= 3 and history[1] == "cooperate" and history[2] == "cooperate" and history[3] == "cooperate" then
        return "defect"
    end
    
    return history[#history]
end
