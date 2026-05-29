-- name: FORGIVER
-- Tit-for-Tat but forgives after 2 defections

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if history[#history] == "defect" then
        -- Check if we've already punished enough
        if #history >= 2 and history[#history - 1] == "defect" then
            -- Forgive
            return "cooperate"
        else
            return "defect"
        end
    else
        return "cooperate"
    end
end
