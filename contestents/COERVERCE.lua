-- name: COERVERCE
-- tries to force cooperation through initial defection

function decide(round, history)
    if round == 1 then return "defect" end
    if round == 2 then return "defect" end
    if round == 3 then return "cooperate" end
    
    if #history >= 3 and history[1] == "cooperate" and history[2] == "cooperate" then
        return "cooperate"
    end
    
    return "defect"
end
