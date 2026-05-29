-- name: COOPERATOR
-- cooperates unless opponent has defected 3 times in a row

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    if #history >= 3 then
        if history[#history] == "defect" and history[#history-1] == "defect" and history[#history-2] == "defect" then
            return "defect"
        end
    end
    
    return "cooperate"
end
