-- name: CONSPIRATOR
-- tries to establish mutual defection then cooperation

function decide(round, history)
    if round <= 3 then return "defect" end
    
    if #history >= 3 then
        local all_defected = true
        for i = 1, 3 do
            if history[i] ~= "defect" then all_defected = false end
        end
        if all_defected then
            return "cooperate"
        end
    end
    
    return "defect"
end
