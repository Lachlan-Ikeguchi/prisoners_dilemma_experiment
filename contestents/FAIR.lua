-- name: FAIR
-- tries to balance cooperation and defection based on opponent's history

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    local their_coop = 0
    for i = 1, #history do
        if history[i] == "cooperate" then
            their_coop = their_coop + 1
        end
    end
    
    -- Cooperate if opponent has cooperated more than 50% of the time
    if their_coop > #history / 2 then
        return "cooperate"
    else
        return "defect"
    end
end
