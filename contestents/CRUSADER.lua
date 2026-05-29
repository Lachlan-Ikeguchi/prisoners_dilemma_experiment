-- name: CRUSADER
-- fights for cooperation

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    local defects = 0
    for i = 1, #history do
        if history[i] == "defect" then defects = defects + 1 end
    end
    
    if defects > 0 then return "defect" end
    return "cooperate"
end
