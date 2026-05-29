-- name: EASY GOING
-- mostly cooperates, defects only if opponent defected twice in a row

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if #history >= 2 and history[#history] == "defect" and history[#history - 1] == "defect" then
        return "defect"
    end
    
    return "cooperate"
end
