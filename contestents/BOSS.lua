-- name: BOSS
-- expects opponent to follow, defects if they don't

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    -- If opponent didn't mirror our last move (assumed cooperate)
    if history[#history] ~= "cooperate" then
        return "defect"
    else
        return "cooperate"
    end
end
