-- name: EYE FOR AN EYE
-- Tit-for-Tat variant that defects twice after one defection

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    -- Count consecutive defections at the end
    local consecutive_defections = 0
    for i = #history, 1, -1 do
        if history[i] == "defect" then
            consecutive_defections = consecutive_defections + 1
        else
            break
        end
    end
    
    -- If opponent has defected consecutively, match their last move
    if consecutive_defections > 0 then
        return "defect"
    end
    
    return "cooperate"
end
