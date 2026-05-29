-- name: MAJORITY
-- plays whatever the opponent has played most often

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
    
    if cooperate_count > defect_count then
        return "cooperate"
    elseif defect_count > cooperate_count then
        return "defect"
    else
        return "cooperate"
    end
end
