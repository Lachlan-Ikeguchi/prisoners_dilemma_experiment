-- name: DOUBTFUL
-- starts with defect, cooperates only after opponent defects

function decide(round, history)
    if round == 1 then return "defect" end
    
    -- Cooperate only if opponent has defected before
    for i = 1, #history do
        if history[i] == "defect" then
            return "cooperate"
        end
    end
    
    return "defect"
end
