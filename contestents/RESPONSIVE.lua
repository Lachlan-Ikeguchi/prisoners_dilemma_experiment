-- name: RESPONSIVE
-- responds to opponent's last two moves

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if #history >= 2 then
        -- If opponent cooperated twice in a row, cooperate
        if history[#history] == "cooperate" and history[#history - 1] == "cooperate" then
            return "cooperate"
        end
        -- If opponent defected twice, defect
        if history[#history] == "defect" and history[#history - 1] == "defect" then
            return "defect"
        end
    end
    
    -- Otherwise mirror last move
    return history[#history]
end
