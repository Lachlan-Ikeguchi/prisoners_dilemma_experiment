-- name: FOOLISH
-- cooperates for first 5 rounds, then uses opponent's most common move

function decide(round, history)
    if round <= 5 then
        return "cooperate"
    else
        local coop = 0
        local defect = 0
        for i = 1, #history do
            if history[i] == "cooperate" then coop = coop + 1
            else defect = defect + 1 end
        end
        
        if coop > defect then return "cooperate"
        else return "defect" end
    end
end
