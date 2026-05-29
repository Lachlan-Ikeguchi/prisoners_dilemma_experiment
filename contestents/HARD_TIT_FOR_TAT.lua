-- name: HARD TIT FOR TAT
-- defects twice after opponent defects once

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if #history >= 1 and history[#history] == "defect" then
        return "defect"
    end
    
    if #history >= 2 and history[#history - 1] == "defect" then
        return "defect"
    end
    
    return "cooperate"
end
