-- name: DEFECTOR
-- defects unless opponent has cooperated 3 times in a row

function decide(round, history)
    if round == 1 then return "defect" end
    
    if #history >= 3 then
        if history[#history] == "cooperate" and history[#history-1] == "cooperate" and history[#history-2] == "cooperate" then
            return "cooperate"
        end
    end
    
    return "defect"
end
