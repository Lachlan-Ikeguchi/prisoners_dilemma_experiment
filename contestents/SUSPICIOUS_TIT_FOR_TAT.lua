-- name: SUSPICIOUS TIT FOR TAT
-- starts by defecting, then mirrors opponent's last move

function decide(round, history)
    -- On first round, defect
    if round == 1 then
        return "defect"
    end
    
    -- Mirror opponent's last move
    if history[#history] == "defect" then
        return "defect"
    else
        return "cooperate"
    end
end
