-- name: TWO TITS FOR TAT
-- defects twice after opponent defects once, then forgives

function decide(round, history)
    -- On first round, cooperate
    if round == 1 then
        return "cooperate"
    end
    
    -- Count recent defections
    local recent_defections = 0
    for i = #history, math.max(1, #history - 1), -1 do
        if history[i] == "defect" then
            recent_defections = recent_defections + 1
        end
    end
    
    -- If opponent defected in last 2 rounds, defect
    if recent_defections > 0 then
        return "defect"
    end
    
    return "cooperate"
end
