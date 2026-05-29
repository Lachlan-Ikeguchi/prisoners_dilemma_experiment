-- name: LOYAL
-- cooperates forever once opponent has cooperated, otherwise defects

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    for i = 1, #history do
        if history[i] == "cooperate" then
            return "cooperate"
        end
    end
    
    return "defect"
end
