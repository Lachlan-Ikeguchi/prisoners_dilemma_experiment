-- name: GRUDGER
-- cooperates until opponent defects, then always defects

function decide(round, history)
    -- On first round, cooperate
    if round == 1 then
        return "cooperate"
    end
    
    -- Check if opponent has ever defected
    for i = 1, #history do
        if history[i] == "defect" then
            return "defect"
        end
    end
    
    return "cooperate"
end
