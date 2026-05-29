-- name: DELAYED RETALIATOR
-- waits one round before retaliating for a defection

function decide(round, history)
    if round == 1 then return "cooperate" end
    if round == 2 then return "cooperate" end
    
    if #history >= 2 then
        if history[#history-1] == "defect" then
            return "defect"
        end
    end
    
    return "cooperate"
end
