-- name: LEADER
-- tries to lead by cooperating, but defects if opponent doesn't follow

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if round == 2 then
        return "cooperate"
    end
    
    -- If opponent didn't cooperate in first two rounds, defect
    if #history >= 2 and history[1] ~= "cooperate" and history[2] ~= "cooperate" then
        return "defect"
    end
    
    -- Otherwise mirror their last move
    return history[#history]
end
