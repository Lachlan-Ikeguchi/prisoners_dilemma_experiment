-- name: FIRST MOVER
-- tries to always be the first to cooperate after mutual defection

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    -- If both defected last round, cooperate to break the cycle
    if #history >= 1 and history[#history] == "defect" then
        if round > 1 then
            -- Estimate our last move (we can't see it, so use a simple pattern)
            -- Just cooperate after they defect
            return "cooperate"
        end
    end
    
    return history[#history]
end
