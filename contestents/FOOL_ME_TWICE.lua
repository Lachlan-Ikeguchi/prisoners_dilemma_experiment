-- name: FOOL ME TWICE
-- cooperates until second defection, then always defects

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
    
    if defect_count >= 2 then
        return "defect"
    else
        return "cooperate"
    end
end
