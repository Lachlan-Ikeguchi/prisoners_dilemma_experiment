-- name: JUDGE
-- cooperates only if opponent has cooperated at least as much as defected

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    local cooperate_count = 0
    local defect_count = 0
    
    for i = 1, #history do
        if history[i] == "cooperate" then
            cooperate_count = cooperate_count + 1
        else
            defect_count = defect_count + 1
        end
    end
    
    if cooperate_count >= defect_count then
        return "cooperate"
    else
        return "defect"
    end
end
