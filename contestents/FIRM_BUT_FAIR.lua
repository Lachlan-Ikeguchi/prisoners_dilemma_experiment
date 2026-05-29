-- name: FIRM BUT FAIR
-- cooperates unless opponent has defected more than once

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    local defect_count = 0
    for i = 1, #history do
        if history[i] == "defect" then
            defect_count = defect_count + 1
        end
    end
    
    if defect_count > 1 then
        return "defect"
    else
        return "cooperate"
    end
end
