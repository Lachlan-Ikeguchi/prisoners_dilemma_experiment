-- name: CONTRITE TIT FOR TAT
-- Tit-for-Tat that tries to apologize after defecting

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    -- If I defected last round (and opponent might be retaliating)
    -- Check if I defected and opponent is now defecting
    if #history >= 2 then
        -- Get my last action from history (this is tricky, we only see opponent's history)
        -- Since we can't see our own history, just use standard TFT
        if history[#history] == "defect" then
            return "defect"
        else
            return "cooperate"
        end
    end
    
    if history[#history] == "defect" then
        return "defect"
    else
        return "cooperate"
    end
end
